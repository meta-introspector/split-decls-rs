macro_rules! deps {
    () => {
        Stack!();
    };
}

macro_rules! Delegate {
    () => {
        deps!();
        # [doc = " A delegate for use in a [`Stack`]."] pub trait Delegate { # [doc = " Called whenever we push a directory on top of the stack, and after the respective call to [`push()`](Self::push)."] # [doc = ""] # [doc = " It is only called if the currently acted on path is a directory in itself, which is determined by knowing"] # [doc = " that it's not the last component of the path."] # [doc = " Use [`Stack::current()`] to see the directory."] fn push_directory (& mut self , stack : & Stack) -> std :: io :: Result < () > ; # [doc = " Called after any component was pushed, with the path available at [`Stack::current()`]."] # [doc = ""] # [doc = " `is_last_component` is `true` if the path is completely built, which typically means it's not a directory."] fn push (& mut self , is_last_component : bool , stack : & Stack) -> std :: io :: Result < () > ; # [doc = " Called right after a directory-component was popped off the stack."] # [doc = ""] # [doc = " Use it to pop information off internal data structures. Note that no equivalent call exists for popping"] # [doc = " the file-component."] fn pop_directory (& mut self) ; }
    };
}

Delegate!()