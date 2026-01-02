mkuse!{use libc :: { c_char , c_uint } ;}
mkuse!{use super :: MetadataKindId ;}
mkuse!{use super :: ffi :: { AttributeKind , BasicBlock , Metadata , Module , Type , Value } ;}
mkuse!{use crate :: llvm :: { Bool , Builder } ;}
mkitem!{# [link (name = "llvm-wrapper" , kind = "static")] unsafe extern "C" { pub (crate) safe fn LLVMRustHasMetadata (I : & Value , KindID : MetadataKindId) -> bool ; pub (crate) fn LLVMRustEraseInstUntilInclusive (BB : & BasicBlock , I : & Value) ; pub (crate) fn LLVMRustGetLastInstruction < 'a > (BB : & BasicBlock) -> Option < & 'a Value > ; pub (crate) fn LLVMRustDIGetInstMetadata (I : & Value) -> Option < & Metadata > ; pub (crate) fn LLVMRustEraseInstFromParent (V : & Value) ; pub (crate) fn LLVMRustGetTerminator < 'a > (B : & BasicBlock) -> & 'a Value ; pub (crate) fn LLVMRustVerifyFunction (V : & Value , action : LLVMRustVerifierFailureAction) -> Bool ; pub (crate) fn LLVMRustHasAttributeAtIndex (V : & Value , i : c_uint , Kind : AttributeKind) -> bool ; pub (crate) fn LLVMRustGetArrayNumElements (Ty : & Type) -> u64 ; pub (crate) fn LLVMRustHasFnAttribute (F : & Value , Name : * const c_char , NameLen : libc :: size_t ,) -> bool ; pub (crate) fn LLVMRustRemoveFnAttribute (F : & Value , Name : * const c_char , NameLen : libc :: size_t) ; pub (crate) fn LLVMGetFirstFunction (M : & Module) -> Option < & Value > ; pub (crate) fn LLVMGetNextFunction (Fn : & Value) -> Option < & Value > ; pub (crate) fn LLVMRustRemoveEnumAttributeAtIndex (Fn : & Value , index : c_uint , kind : AttributeKind ,) ; pub (crate) fn LLVMRustPositionBefore < 'a > (B : & 'a Builder < '_ > , I : & 'a Value) ; pub (crate) fn LLVMRustPositionAfter < 'a > (B : & 'a Builder < '_ > , I : & 'a Value) ; pub (crate) fn LLVMRustGetFunctionCall (F : & Value , name : * const c_char , NameLen : libc :: size_t ,) -> Option < & Value > ; }}
mkitem!{unsafe extern "C" { pub (crate) fn LLVMDumpModule (M : & Module) ; pub (crate) fn LLVMDumpValue (V : & Value) ; pub (crate) fn LLVMGetFunctionCallConv (F : & Value) -> c_uint ; pub (crate) fn LLVMGetReturnType (T : & Type) -> & Type ; pub (crate) fn LLVMGetParams (Fnc : & Value , params : * mut & Value) ; pub (crate) fn LLVMGetNamedFunction (M : & Module , Name : * const c_char) -> Option < & Value > ; }}
mkitem!{mkenum!{# [repr (C)] # [derive (Copy , Clone , PartialEq)] pub (crate) enum LLVMRustVerifierFailureAction { LLVMAbortProcessAction = 0 , LLVMPrintMessageAction = 1 , LLVMReturnStatusAction = 2 , }}}
mkuse!{# [cfg (llvm_enzyme)] pub (crate) use self :: Enzyme_AD :: * ;}
mkmod!{Enzyme_AD, { 
                getname!(Enzyme_AD);
                getsrc!(Enzyme_AD);
                getpath!(Enzyme_AD);
                get_deps!(Enzyme_AD);
                get_crates!(Enzyme_AD);
                mkinclude!(Enzyme_AD);
                mkuse!{use std :: ffi :: { CString , c_char } ;}
mkuse!{use libc :: c_void ;}
mkitem!{unsafe extern "C" { pub (crate) fn EnzymeSetCLBool (arg1 : * mut :: std :: os :: raw :: c_void , arg2 : u8) ; pub (crate) fn EnzymeSetCLString (arg1 : * mut :: std :: os :: raw :: c_void , arg2 : * const c_char) ; }}
mkitem!{unsafe extern "C" { static mut EnzymePrintPerf : c_void ; static mut EnzymePrintActivity : c_void ; static mut EnzymePrintType : c_void ; static mut EnzymeFunctionToAnalyze : c_void ; static mut EnzymePrint : c_void ; static mut EnzymeStrictAliasing : c_void ; static mut looseTypeAnalysis : c_void ; static mut EnzymeInline : c_void ; static mut RustTypeRules : c_void ; }}

macro_rules! set_print_perf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_print_perf in module {}", module_path!());
    };
}

mkfn!{
    set_print_perf_introspect!();
    pub (crate) fn set_print_perf (print : bool) { unsafe { EnzymeSetCLBool (std :: ptr :: addr_of_mut ! (EnzymePrintPerf) , print as u8) ; } }
}

macro_rules! set_print_activity_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_print_activity in module {}", module_path!());
    };
}

mkfn!{
    set_print_activity_introspect!();
    pub (crate) fn set_print_activity (print : bool) { unsafe { EnzymeSetCLBool (std :: ptr :: addr_of_mut ! (EnzymePrintActivity) , print as u8) ; } }
}

macro_rules! set_print_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_print_type in module {}", module_path!());
    };
}

mkfn!{
    set_print_type_introspect!();
    pub (crate) fn set_print_type (print : bool) { unsafe { EnzymeSetCLBool (std :: ptr :: addr_of_mut ! (EnzymePrintType) , print as u8) ; } }
}

macro_rules! set_print_type_fun_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_print_type_fun in module {}", module_path!());
    };
}

mkfn!{
    set_print_type_fun_introspect!();
    pub (crate) fn set_print_type_fun (fun_name : & str) { let c_fun_name = CString :: new (fun_name) . unwrap () ; unsafe { EnzymeSetCLString (std :: ptr :: addr_of_mut ! (EnzymeFunctionToAnalyze) , c_fun_name . as_ptr () as * const c_char ,) ; } }
}

macro_rules! set_print_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_print in module {}", module_path!());
    };
}

mkfn!{
    set_print_introspect!();
    pub (crate) fn set_print (print : bool) { unsafe { EnzymeSetCLBool (std :: ptr :: addr_of_mut ! (EnzymePrint) , print as u8) ; } }
}

macro_rules! set_strict_aliasing_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_strict_aliasing in module {}", module_path!());
    };
}

mkfn!{
    set_strict_aliasing_introspect!();
    pub (crate) fn set_strict_aliasing (strict : bool) { unsafe { EnzymeSetCLBool (std :: ptr :: addr_of_mut ! (EnzymeStrictAliasing) , strict as u8) ; } }
}

macro_rules! set_loose_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_loose_types in module {}", module_path!());
    };
}

mkfn!{
    set_loose_types_introspect!();
    pub (crate) fn set_loose_types (loose : bool) { unsafe { EnzymeSetCLBool (std :: ptr :: addr_of_mut ! (looseTypeAnalysis) , loose as u8) ; } }
}

macro_rules! set_inline_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_inline in module {}", module_path!());
    };
}

mkfn!{
    set_inline_introspect!();
    pub (crate) fn set_inline (val : bool) { unsafe { EnzymeSetCLBool (std :: ptr :: addr_of_mut ! (EnzymeInline) , val as u8) ; } }
}

macro_rules! set_rust_rules_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_rust_rules in module {}", module_path!());
    };
}

mkfn!{
    set_rust_rules_introspect!();
    pub (crate) fn set_rust_rules (val : bool) { unsafe { EnzymeSetCLBool (std :: ptr :: addr_of_mut ! (RustTypeRules) , val as u8) ; } }
} 
            }}
mkuse!{# [cfg (not (llvm_enzyme))] pub (crate) use self :: Fallback_AD :: * ;}
mkmod!{Fallback_AD, { 
                getname!(Fallback_AD);
                getsrc!(Fallback_AD);
                getpath!(Fallback_AD);
                get_deps!(Fallback_AD);
                get_crates!(Fallback_AD);
                mkinclude!(Fallback_AD);
                
macro_rules! set_inline_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_inline in module {}", module_path!());
    };
}

mkfn!{
    set_inline_introspect!();
    pub (crate) fn set_inline (val : bool) { unimplemented ! () }
}

macro_rules! set_print_perf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_print_perf in module {}", module_path!());
    };
}

mkfn!{
    set_print_perf_introspect!();
    pub (crate) fn set_print_perf (print : bool) { unimplemented ! () }
}

macro_rules! set_print_activity_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_print_activity in module {}", module_path!());
    };
}

mkfn!{
    set_print_activity_introspect!();
    pub (crate) fn set_print_activity (print : bool) { unimplemented ! () }
}

macro_rules! set_print_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_print_type in module {}", module_path!());
    };
}

mkfn!{
    set_print_type_introspect!();
    pub (crate) fn set_print_type (print : bool) { unimplemented ! () }
}

macro_rules! set_print_type_fun_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_print_type_fun in module {}", module_path!());
    };
}

mkfn!{
    set_print_type_fun_introspect!();
    pub (crate) fn set_print_type_fun (fun_name : & str) { unimplemented ! () }
}

macro_rules! set_print_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_print in module {}", module_path!());
    };
}

mkfn!{
    set_print_introspect!();
    pub (crate) fn set_print (print : bool) { unimplemented ! () }
}

macro_rules! set_strict_aliasing_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_strict_aliasing in module {}", module_path!());
    };
}

mkfn!{
    set_strict_aliasing_introspect!();
    pub (crate) fn set_strict_aliasing (strict : bool) { unimplemented ! () }
}

macro_rules! set_loose_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_loose_types in module {}", module_path!());
    };
}

mkfn!{
    set_loose_types_introspect!();
    pub (crate) fn set_loose_types (loose : bool) { unimplemented ! () }
}

macro_rules! set_rust_rules_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_rust_rules in module {}", module_path!());
    };
}

mkfn!{
    set_rust_rules_introspect!();
    pub (crate) fn set_rust_rules (val : bool) { unimplemented ! () }
} 
            }}