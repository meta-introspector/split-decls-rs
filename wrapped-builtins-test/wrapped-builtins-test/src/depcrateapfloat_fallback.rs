// Generated macro for apfloat_fallback (macro)
macro_rules! Depcrateapfloat_fallback {
() => {
// Module: crate
// Provides: {"apfloat_fallback"}
// Dependencies: {}
# [doc = " Perform an operation using builtin types if available, falling back to apfloat if not."] # [macro_export] macro_rules ! apfloat_fallback { ($ float_ty : ty , $ apfloat_ty : ident , $ sys_available : meta , $ op : expr $ (=> $ convert : ident) ? $ (; $ apfloat_op : expr) ?, $ ($ arg : expr) ,+ $ (,) ?) => { { # [cfg ($ sys_available)] let ret = { type FloatTy = $ float_ty ; $ op ($ ($ arg) ,+) } ; # [cfg (not ($ sys_available))] let ret = { use rustc_apfloat :: Float ; type FloatTy = rustc_apfloat :: ieee ::$ apfloat_ty ; apfloat_fallback ! (@ inner fty : $ float_ty , op_res : $ op ($ (FloatTy :: from_bits ($ arg . to_bits () . into ())) ,+) , $ (apfloat_op : $ apfloat_op ,) ? $ (conv_opts : $ convert ,) ? args : $ ($ arg) ,+) } ; ret } } ; (@ inner fty : $ float_ty : ty , op_res : $ val : expr , conv_opts : no_convert , args : $ ($ _arg : expr) ,+) => { $ val } ; (@ inner fty : $ float_ty : ty , op_res : $ val : expr , args : $ ($ _arg : expr) ,+) => { { let unwrapped = $ val . value ; <$ float_ty >:: from_bits (FloatTy :: to_bits (unwrapped) . try_into () . unwrap ()) } } ; (@ inner fty : $ float_ty : ty , op_res : $ _val : expr , apfloat_op : $ apfloat_op : expr , args : $ ($ arg : expr) ,+) => { { $ apfloat_op ($ ($ arg) ,+) } } ; }
};
}
