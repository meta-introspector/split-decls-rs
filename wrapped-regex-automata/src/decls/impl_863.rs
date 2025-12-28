macro_rules! deps {
    () => {
        StateBuilderNFA!();
        LookSet!();
        Repr!();
        StateID!();
        ReprVec!();
        State!();
        StateBuilderEmpty!();
    };
}

macro_rules! impl_863 {
    () => {
        deps!();
        # [doc = " For docs on these routines, see the internal Repr and ReprVec types below."] impl StateBuilderNFA { pub (crate) fn to_state (& self) -> State { State (Arc :: from (& * self . repr)) } pub (crate) fn clear (self) -> StateBuilderEmpty { let mut builder = StateBuilderEmpty (self . repr) ; builder . clear () ; builder } pub (crate) fn look_need (& self) -> LookSet { self . repr () . look_need () } pub (crate) fn set_look_have (& mut self , set : impl FnMut (LookSet) -> LookSet ,) { self . repr_vec () . set_look_have (set) } pub (crate) fn set_look_need (& mut self , set : impl FnMut (LookSet) -> LookSet ,) { self . repr_vec () . set_look_need (set) } pub (crate) fn add_nfa_state_id (& mut self , sid : StateID) { ReprVec (& mut self . repr) . add_nfa_state_id (& mut self . prev_nfa_state_id , sid) } pub (crate) fn as_bytes (& self) -> & [u8] { & self . repr } fn repr (& self) -> Repr < '_ > { Repr (& self . repr) } fn repr_vec (& mut self) -> ReprVec < '_ > { ReprVec (& mut self . repr) } }
    };
}

impl_863!()