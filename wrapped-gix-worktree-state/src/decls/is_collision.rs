macro_rules! deps {
    () => {
        Error!();
        Collision!();
    };
}

macro_rules! is_collision {
    () => {
        deps!();
        fn is_collision (err : & std :: io :: Error , entry_path : & BStr , collisions : & mut Vec < checkout :: Collision > , files : & AtomicUsize ,) -> bool { if ! gix_fs :: symlink :: is_collision_error (err) { return false ; } gix_features :: trace :: error ! ("{entry_path}: collided ({:?})" , err . kind ()) ; collisions . push (checkout :: Collision { path : entry_path . into () , error_kind : err . kind () , }) ; files . fetch_add (1 , Ordering :: Relaxed) ; true }
    };
}

is_collision!();