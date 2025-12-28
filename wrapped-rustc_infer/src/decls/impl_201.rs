macro_rules! deps {
    () => {
        UndoLog!();
        InferCtxtUndoLogs!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        # [doc = " The UndoLogs trait defines how we undo a particular kind of action (of type T). We can undo any"] # [doc = " action that is convertible into an UndoLog (per the From impls above)."] impl < 'tcx , T > UndoLogs < T > for InferCtxtUndoLogs < 'tcx > where UndoLog < 'tcx > : From < T > , { # [inline] fn num_open_snapshots (& self) -> usize { self . num_open_snapshots } # [inline] fn push (& mut self , undo : T) { if self . in_snapshot () { self . logs . push (undo . into ()) } } fn clear (& mut self) { self . logs . clear () ; self . num_open_snapshots = 0 ; } fn extend < J > (& mut self , undos : J) where Self : Sized , J : IntoIterator < Item = T > , { if self . in_snapshot () { self . logs . extend (undos . into_iter () . map (UndoLog :: from)) } } }
    };
}

impl_201!();