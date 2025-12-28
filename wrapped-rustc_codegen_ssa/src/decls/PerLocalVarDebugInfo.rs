macro_rules! PerLocalVarDebugInfo {
    () => {
        # [doc = " Like `mir::VarDebugInfo`, but within a `mir::Local`."] # [derive (Clone)] pub struct PerLocalVarDebugInfo < 'tcx , D > { pub name : Symbol , pub source_info : mir :: SourceInfo , # [doc = " `DIVariable` returned by `create_dbg_var`."] pub dbg_var : Option < D > , # [doc = " Byte range in the `dbg_var` covered by this fragment,"] # [doc = " if this is a fragment of a composite `VarDebugInfo`."] pub fragment : Option < Range < Size > > , # [doc = " `.place.projection` from `mir::VarDebugInfo`."] pub projection : & 'tcx ty :: List < mir :: PlaceElem < 'tcx > > , }
    };
}

PerLocalVarDebugInfo!()