macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! message_different {
    () => {
        deps!();
        # [doc = " Prints the difference of the two snippets of expanded code."] pub (crate) fn message_different (name : & str , a : & [u8] , b : & [u8]) { let a = String :: from_utf8_lossy (a) ; let b = String :: from_utf8_lossy (b) ; let changes = diff :: lines (& a , & b) ; let mut lines_added = 0 ; let mut lines_removed = 0 ; for diff in & changes { match diff { Result :: Left (_) => lines_added += 1 , Result :: Right (_) => lines_removed += 1 , _ => () , } } eprintln ! ("{} - different!" , name) ; eprintln ! ("Diff [lines: {} added, {} removed]:" , lines_added , lines_removed) ; eprintln ! ("--------------------------") ; for change in changes { match change { Result :: Both (x , _) => { eprintln ! (" {}" , x) ; } Result :: Left (x) => { eprintln ! ("+{}" , x) ; } Result :: Right (x) => { eprintln ! ("-{}" , x) ; } } } eprintln ! ("--------------------------") ; }
    };
}

message_different!();