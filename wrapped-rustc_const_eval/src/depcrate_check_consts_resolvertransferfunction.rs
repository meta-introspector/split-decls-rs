// Generated macro for TransferFunction (struct)
macro_rules! Depcrate_check_consts_resolverTransferFunction {
() => {
// Module: crate::check_consts::resolver
// Provides: {"TransferFunction"}
// Dependencies: {}
# [doc = " A `Visitor` that propagates qualifs between locals. This defines the transfer function of"] # [doc = " `FlowSensitiveAnalysis`."] # [doc = ""] # [doc = " To account for indirect assignments, data flow conservatively assumes that local becomes"] # [doc = " qualified immediately after it is borrowed or its address escapes. The borrow must allow for"] # [doc = " mutation, which includes shared borrows of places with interior mutability. The type of"] # [doc = " borrowed place must contain the qualif."] struct TransferFunction < 'mir , 'tcx , Q > { ccx : & 'mir ConstCx < 'mir , 'tcx > , state : & 'mir mut State , _qualif : PhantomData < Q > , }
};
}
