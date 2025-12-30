// Generated macro for bench_update (macro)
macro_rules! Depcrate_devbench_update {
() => {
// Module: crate::dev
// Provides: {"bench_update"}
// Dependencies: {}
# [doc = " Define [`Update`][crate::Update] impl benchmark"] # [macro_export] macro_rules ! bench_update { ($ init : expr ; $ ($ name : ident $ bs : expr ;) *) => { $ (# [bench] fn $ name (b : & mut Bencher) { let mut d = $ init ; let data = [0 ; $ bs] ; b . iter (|| { digest :: Update :: update (& mut d , & data [..]) ; }) ; b . bytes = $ bs ; }) * } ; }
};
}
