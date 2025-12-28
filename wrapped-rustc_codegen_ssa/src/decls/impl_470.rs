macro_rules! deps {
    () => {
        DebugScope!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl < 'tcx , S : Copy , L : Copy > DebugScope < S , L > { # [doc = " DILocations inherit source file name from the parent DIScope. Due to macro expansions"] # [doc = " it may so happen that the current span belongs to a different file than the DIScope"] # [doc = " corresponding to span's containing source scope. If so, we need to create a DIScope"] # [doc = " \"extension\" into that file."] pub fn adjust_dbg_scope_for_span < Cx : CodegenMethods < 'tcx , DIScope = S , DILocation = L > > (& self , cx : & Cx , span : Span ,) -> S { let pos = span . lo () ; if pos < self . file_start_pos || pos >= self . file_end_pos { let sm = cx . sess () . source_map () ; cx . extend_scope_to_file (self . dbg_scope , & sm . lookup_char_pos (pos) . file) } else { self . dbg_scope } } }
    };
}

impl_470!()