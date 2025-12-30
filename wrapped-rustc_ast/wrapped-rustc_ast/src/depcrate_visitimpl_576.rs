// Generated macro for impl_576 (impl)
macro_rules! Depcrate_visitimpl_576 {
() => {
// Module: crate::visit
// Provides: {"impl_576"}
// Dependencies: {}
impl < 'a , V : Visitor < 'a > , T1 , T2 , T3 > Visitable < 'a , V > for (T1 , T2 , T3) where T1 : Visitable < 'a , V , Extra = () > , T2 : Visitable < 'a , V , Extra = () > , T3 : Visitable < 'a , V , Extra = () > , { type Extra = () ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { try_visit ! (self . 0 . visit (visitor , extra)) ; try_visit ! (self . 1 . visit (visitor , extra)) ; try_visit ! (self . 2 . visit (visitor , extra)) ; V :: Result :: output () } }
};
}
