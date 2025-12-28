macro_rules! deps {
    () => {
        Immediate!();
        LocalValue!();
        LocalState!();
        MemPlaceMeta!();
        MemPlace!();
        Operand!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > LocalState < 'tcx , Prov > { pub fn make_live_uninit (& mut self) { self . value = LocalValue :: Live (Operand :: Immediate (Immediate :: Uninit)) ; } # [doc = " This is a hack because Miri needs a way to visit all the provenance in a `LocalState`"] # [doc = " without having a layout or `TyCtxt` available, and we want to keep the `Operand` type"] # [doc = " private."] pub fn as_mplace_or_imm (& self ,) -> Option < Either < (Pointer < Option < Prov > > , MemPlaceMeta < Prov >) , Immediate < Prov > > > { match self . value { LocalValue :: Dead => None , LocalValue :: Live (Operand :: Indirect (mplace)) => Some (Left ((mplace . ptr , mplace . meta))) , LocalValue :: Live (Operand :: Immediate (imm)) => Some (Right (imm)) , } } # [doc = " Read the local's value or error if the local is not yet live or not live anymore."] # [inline (always)] pub (super) fn access (& self) -> InterpResult < 'tcx , & Operand < Prov > > { match & self . value { LocalValue :: Dead => throw_ub ! (DeadLocal) , LocalValue :: Live (val) => interp_ok (val) , } } # [doc = " Overwrite the local. If the local can be overwritten in place, return a reference"] # [doc = " to do so; otherwise return the `MemPlace` to consult instead."] # [inline (always)] pub (super) fn access_mut (& mut self) -> InterpResult < 'tcx , & mut Operand < Prov > > { match & mut self . value { LocalValue :: Dead => throw_ub ! (DeadLocal) , LocalValue :: Live (val) => interp_ok (val) , } } }
    };
}

impl_320!()