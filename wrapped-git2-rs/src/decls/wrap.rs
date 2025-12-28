macro_rules! wrap {
    () => {
        pub fn wrap < T , F : FnOnce () -> T + std :: panic :: UnwindSafe > (f : F) -> Option < T > { use std :: panic ; if LAST_ERROR . with (| slot | slot . borrow () . is_some ()) { return None ; } match panic :: catch_unwind (f) { Ok (ret) => Some (ret) , Err (e) => { LAST_ERROR . with (move | slot | { * slot . borrow_mut () = Some (e) ; }) ; None } } }
    };
}

wrap!();