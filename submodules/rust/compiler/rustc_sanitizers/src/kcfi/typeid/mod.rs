mkuse!{use std :: hash :: Hasher ;}
mkuse!{use rustc_middle :: ty :: { Instance , InstanceKind , ReifyReason , Ty , TyCtxt } ;}
mkuse!{use rustc_target :: callconv :: FnAbi ;}
mkuse!{use twox_hash :: XxHash64 ;}
mkuse!{pub use crate :: cfi :: typeid :: { TypeIdOptions , itanium_cxx_abi } ;}

macro_rules! typeid_for_fnabi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function typeid_for_fnabi in module {}", module_path!());
    };
}

mkfn!{
    typeid_for_fnabi_introspect!();
    # [doc = " Returns a KCFI type metadata identifier for the specified FnAbi."] pub fn typeid_for_fnabi < 'tcx > (tcx : TyCtxt < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , options : TypeIdOptions ,) -> u32 { let mut hash : XxHash64 = Default :: default () ; hash . write (itanium_cxx_abi :: typeid_for_fnabi (tcx , fn_abi , options) . as_bytes ()) ; hash . finish () as u32 }
}

macro_rules! typeid_for_instance_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function typeid_for_instance in module {}", module_path!());
    };
}

mkfn!{
    typeid_for_instance_introspect!();
    # [doc = " Returns a KCFI type metadata identifier for the specified Instance."] pub fn typeid_for_instance < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , mut options : TypeIdOptions ,) -> u32 { if matches ! (instance . def , InstanceKind :: ReifyShim (_ , Some (ReifyReason :: FnPtr))) { options . insert (TypeIdOptions :: USE_CONCRETE_SELF) ; } let mut hash : XxHash64 = Default :: default () ; hash . write (itanium_cxx_abi :: typeid_for_instance (tcx , instance , options) . as_bytes ()) ; hash . finish () as u32 }
}