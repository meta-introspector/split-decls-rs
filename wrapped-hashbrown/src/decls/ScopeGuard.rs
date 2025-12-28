macro_rules! ScopeGuard {
    () => {
        pub struct ScopeGuard < T , F > where F : FnMut (& mut T) , { dropfn : F , value : T , }
    };
}

ScopeGuard!();