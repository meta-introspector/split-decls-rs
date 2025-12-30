// Generated macro for impl_21 (impl)
macro_rules! Depcrate_nestedimpl_21 {
() => {
// Module: crate::nested
// Provides: {"impl_21"}
// Dependencies: {}
impl Node { # [doc = " Create a new node."] fn new () -> Self { Node { children : vec ! [] } } fn new_rc_refcell () -> Rc < RefCell < Self > > { Rc :: new (RefCell :: new (Self :: new ())) } # [doc = " Append a child to the node."] fn push_child (& mut self , child : Rc < RefCell < Self > >) { self . children . push (child) ; } # [doc = " Write the YAML representation of the node to `writer`."] fn write_to < W : std :: io :: Write > (& self , writer : & mut W , indent : usize) -> std :: io :: Result < () > { if self . children . is_empty () { write_n (writer , ' ' , indent) ? ; writer . write_all (b"a: 1\n") ? ; } else { for (n , child) in self . children . iter () . enumerate () { write_n (writer , ' ' , indent) ? ; write_id_for_number (writer , n) ? ; writer . write_all (b":\n") ? ; (* * child) . borrow () . write_to (writer , indent + 2) ? ; } } Ok (()) } }
};
}
