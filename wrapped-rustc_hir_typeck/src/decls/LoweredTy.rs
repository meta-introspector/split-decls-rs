macro_rules! LoweredTy {
    () => {
        # [doc = " The `ty` representation of a user-provided type. Depending on the use-site"] # [doc = " we want to either use the unnormalized or the normalized form of this type."] # [doc = ""] # [doc = " This is a bridge between the interface of HIR ty lowering, which outputs a raw"] # [doc = " `Ty`, and the API in this module, which expect `Ty` to be fully normalized."] # [derive (Clone , Copy , Debug)] pub (crate) struct LoweredTy < 'tcx > { # [doc = " The unnormalized type provided by the user."] pub raw : Ty < 'tcx > , # [doc = " The normalized form of `raw`, stored here for efficiency."] pub normalized : Ty < 'tcx > , }
    };
}

LoweredTy!()