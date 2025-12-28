macro_rules! deps {
    () => {
        ScopeGuard!();
    };
}

macro_rules! guard {
    () => {
        deps!();
        # [inline] pub fn guard < T , F > (value : T , dropfn : F) -> ScopeGuard < T , F > where F : FnMut (& mut T) , { ScopeGuard { dropfn , value } }
    };
}

guard!();