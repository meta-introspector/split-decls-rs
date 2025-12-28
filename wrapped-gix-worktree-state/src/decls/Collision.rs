macro_rules! Collision {
    () => {
        # [doc = " Information about a path that failed to checkout as something else was already present."] # [derive (Clone , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct Collision { # [doc = " the path that collided with something already present on disk."] pub path : BString , # [doc = " The io error we encountered when checking out `path`."] pub error_kind : std :: io :: ErrorKind , }
    };
}

Collision!()