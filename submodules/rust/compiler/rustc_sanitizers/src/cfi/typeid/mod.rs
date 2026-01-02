mkuse!{use bitflags :: bitflags ;}
mkuse!{use rustc_middle :: ty :: { Instance , Ty , TyCtxt } ;}
mkuse!{use rustc_target :: callconv :: FnAbi ;}
mkitem!{bitflags ! { # [doc = " Options for typeid_for_fnabi."] # [derive (Clone , Copy , Debug)] pub struct TypeIdOptions : u32 { # [doc = " Generalizes pointers for compatibility with Clang"] # [doc = " `-fsanitize-cfi-icall-generalize-pointers` option for cross-language LLVM CFI and KCFI"] # [doc = " support."] const GENERALIZE_POINTERS = 1 ; # [doc = " Generalizes repr(C) user-defined type for extern function types with the \"C\" calling"] # [doc = " convention (or extern types) for cross-language LLVM CFI and  KCFI support."] const GENERALIZE_REPR_C = 2 ; # [doc = " Normalizes integers for compatibility with Clang"] # [doc = " `-fsanitize-cfi-icall-experimental-normalize-integers` option for cross-language LLVM"] # [doc = " CFI and  KCFI support."] const NORMALIZE_INTEGERS = 4 ; # [doc = " Do not perform self type erasure for attaching a secondary type id to methods with their"] # [doc = " concrete self so they can be used as function pointers."] # [doc = ""] # [doc = " (This applies to typeid_for_instance only and should be used to attach a secondary type"] # [doc = " id to methods during their declaration/definition so they match the type ids returned by"] # [doc = " either typeid_for_instance or typeid_for_fnabi at call sites during code generation for"] # [doc = " type membership tests when methods are used as function pointers.)"] const USE_CONCRETE_SELF = 8 ; } }}
mkmod!{itanium_cxx_abi, { 
                getname!(itanium_cxx_abi);
                getsrc!(itanium_cxx_abi);
                getpath!(itanium_cxx_abi);
                get_deps!(itanium_cxx_abi);
                get_crates!(itanium_cxx_abi);
                mkinclude!(itanium_cxx_abi);
                 
            }}

macro_rules! typeid_for_fnabi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function typeid_for_fnabi in module {}", module_path!());
    };
}

mkfn!{
    typeid_for_fnabi_introspect!();
    # [doc = " Returns a type metadata identifier for the specified FnAbi."] pub fn typeid_for_fnabi < 'tcx > (tcx : TyCtxt < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , options : TypeIdOptions ,) -> String { itanium_cxx_abi :: typeid_for_fnabi (tcx , fn_abi , options) }
}

macro_rules! typeid_for_instance_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function typeid_for_instance in module {}", module_path!());
    };
}

mkfn!{
    typeid_for_instance_introspect!();
    # [doc = " Returns a type metadata identifier for the specified Instance."] pub fn typeid_for_instance < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , options : TypeIdOptions ,) -> String { itanium_cxx_abi :: typeid_for_instance (tcx , instance , options) }
}