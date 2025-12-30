// Generated macro for write_lines (function)
macro_rules! Depcrate_printerwrite_lines {
() => {
// Module: crate::printer
// Provides: {"write_lines"}
// Dependencies: {}
# [doc = " Present the diff output for two mutliline strings in a pretty, colorised manner."] pub (crate) fn write_lines < TWrite : fmt :: Write > (f : & mut TWrite , left : & str , right : & str ,) -> fmt :: Result { let diff = :: diff :: lines (left , right) ; let mut changes = diff . into_iter () . peekable () ; let mut previous_deletion = LatentDeletion :: default () ; while let Some (change) = changes . next () { match (change , changes . peek ()) { (:: diff :: Result :: Both (value , _) , _) => { previous_deletion . flush (f) ? ; writeln ! (f , " {}" , value) ? ; } (:: diff :: Result :: Left (deleted) , _) => { previous_deletion . flush (f) ? ; previous_deletion . set (deleted) ; } (:: diff :: Result :: Right (inserted) , Some (:: diff :: Result :: Right (_))) => { previous_deletion . flush (f) ? ; paint ! (f , Green , "{}{}" , SIGN_RIGHT , inserted) ? ; writeln ! (f) ? ; } (:: diff :: Result :: Right (inserted) , _) => { if let Some (deleted) = previous_deletion . take () { write_inline_diff (f , deleted , inserted) ? ; } else { previous_deletion . flush (f) ? ; paint ! (f , Green , "{}{}" , SIGN_RIGHT , inserted) ? ; writeln ! (f) ? ; } } } ; } previous_deletion . flush (f) ? ; Ok (()) }
};
}
