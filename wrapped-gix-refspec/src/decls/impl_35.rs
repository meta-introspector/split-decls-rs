macro_rules! deps {
    () => {
        Issue!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl std :: fmt :: Display for Issue { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Issue :: Conflict { destination_full_ref_name , sources , specs , } => { write ! (f , "Conflicting destination {destination_full_ref_name:?} would be written by {}" , sources . iter () . zip (specs . iter ()) . map (| (src , spec) | format ! ("{src} ({spec:?})")) . collect ::< Vec < _ >> () . join (", ")) } } } }
    };
}

impl_35!()