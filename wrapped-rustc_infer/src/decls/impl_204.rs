macro_rules! deps {
    () => {
        InferCtxtUndoLogs!();
        UndoLog!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < 'tcx > std :: ops :: Index < usize > for InferCtxtUndoLogs < 'tcx > { type Output = UndoLog < 'tcx > ; fn index (& self , key : usize) -> & Self :: Output { & self . logs [key] } }
    };
}

impl_204!()