macro_rules! deps {
    () => {
        StartError!();
        Cache!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for StartError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match * self { StartError :: Cache { ref err } => Some (err) , _ => None , } } }
    };
}

impl_259!()