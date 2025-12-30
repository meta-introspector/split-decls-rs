// Generated macro for impl_48 (impl)
macro_rules! Depcrate_dir_createimpl_48 {
() => {
// Module: crate::dir::create
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = Result < & 'a Path , Error < 'a > > ; fn next (& mut self) -> Option < Self :: Item > { use std :: io :: ErrorKind :: * ; match self . cursors . pop () { Some (dir) => match std :: fs :: create_dir (dir) { Ok (()) => { self . state = State :: CurrentlyCreatingDirectories ; Some (Ok (dir)) } Err (err) => match err . kind () { AlreadyExists if dir . is_dir () => { self . state = State :: CurrentlyCreatingDirectories ; Some (Ok (dir)) } AlreadyExists => self . permanent_failure (dir , err) , NotFound => { self . retries . on_create_directory_failure -= 1 ; if let State :: CurrentlyCreatingDirectories = self . state { self . state = State :: SearchingUpwardsForExistingDirectory ; self . retries . to_create_entire_directory -= 1 ; if self . retries . to_create_entire_directory < 1 { return self . permanent_failure (dir , NotFound) ; } self . retries . on_create_directory_failure = self . original_retries . on_create_directory_failure ; } if self . retries . on_create_directory_failure < 1 { return self . permanent_failure (dir , NotFound) ; } self . cursors . push (dir) ; self . cursors . push (match dir . parent () { None => return self . permanent_failure (dir , InvalidInput) , Some (parent) => parent , }) ; self . intermediate_failure (dir , err) } Interrupted => { self . retries . on_interrupt -= 1 ; if self . retries . on_interrupt <= 1 { return self . permanent_failure (dir , Interrupted) ; } self . cursors . push (dir) ; self . intermediate_failure (dir , err) } _unexpected_kind => self . permanent_failure (dir , err) , } , } , None => None , } } }
};
}
