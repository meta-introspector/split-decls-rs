macro_rules! deps {
    () => {
        Fallibility!();
        RawTableInner!();
        RawTable!();
        ScopeGuard!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < T : Clone , A : Allocator + Clone > Clone for RawTable < T , A > { fn clone (& self) -> Self { if self . table . is_empty_singleton () { Self :: new_in (self . alloc . clone ()) } else { unsafe { let mut new_table = match Self :: new_uninitialized (self . alloc . clone () , self . table . buckets () , Fallibility :: Infallible ,) { Ok (table) => table , Err (_) => hint :: unreachable_unchecked () , } ; new_table . clone_from_spec (self) ; new_table } } } fn clone_from (& mut self , source : & Self) { if source . table . is_empty_singleton () { let mut old_inner = mem :: replace (& mut self . table , RawTableInner :: NEW) ; unsafe { old_inner . drop_inner_table :: < T , _ > (& self . alloc , Self :: TABLE_LAYOUT) ; } } else { unsafe { let mut self_ = guard (self , | self_ | { self_ . clear_no_drop () ; }) ; self_ . table . drop_elements :: < T > () ; if self_ . buckets () != source . buckets () { let new_inner = match RawTableInner :: new_uninitialized (& self_ . alloc , Self :: TABLE_LAYOUT , source . buckets () , Fallibility :: Infallible ,) { Ok (table) => table , Err (_) => hint :: unreachable_unchecked () , } ; let mut old_inner = mem :: replace (& mut self_ . table , new_inner) ; if ! old_inner . is_empty_singleton () { old_inner . free_buckets (& self_ . alloc , Self :: TABLE_LAYOUT) ; } } self_ . clone_from_spec (source) ; ScopeGuard :: into_inner (self_) ; } } } }
    };
}

impl_59!()