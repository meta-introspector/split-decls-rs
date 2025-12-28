macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Error for OpenError { fn source (& self) -> Option < & (dyn Error + 'static) > { match self { OpenError :: Io (inner) => Some (inner) , OpenError :: Spawn { cmds : _ , source } => Some (source) , OpenError :: ExitStatus { .. } => None , } } }
    };
}

impl_9!()