macro_rules! deps {
    () => {
        InferCtxtUndoLogs!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < 'tcx > std :: ops :: IndexMut < usize > for InferCtxtUndoLogs < 'tcx > { fn index_mut (& mut self , key : usize) -> & mut Self :: Output { & mut self . logs [key] } }
    };
}

impl_205!()