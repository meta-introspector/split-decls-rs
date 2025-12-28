macro_rules! do_op {
    () => {
        fn do_op < F > (path : & Path , desc : & str , mut f : F) where F : FnMut (& Path) -> io :: Result < () > , { match f (path) { Ok (()) => { } Err (ref e) if e . kind () == ErrorKind :: PermissionDenied => { let mut p = t ! (path . metadata ()) . permissions () ; p . set_readonly (false) ; t ! (fs :: set_permissions (path , p)) ; let parent = path . parent () . unwrap () ; let mut p = t ! (parent . metadata ()) . permissions () ; p . set_readonly (false) ; t ! (fs :: set_permissions (parent , p)) ; f (path) . unwrap_or_else (| e | { panic ! ("failed to {} {}: {}" , desc , path . display () , e) ; }) } Err (e) => { panic ! ("failed to {} {}: {}" , desc , path . display () , e) ; } } }
    };
}

do_op!()