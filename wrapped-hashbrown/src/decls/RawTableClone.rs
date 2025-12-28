macro_rules! RawTableClone {
    () => {
        # [doc = " Specialization of `clone_from` for `Copy` types"] trait RawTableClone { unsafe fn clone_from_spec (& mut self , source : & Self) ; }
    };
}

RawTableClone!()