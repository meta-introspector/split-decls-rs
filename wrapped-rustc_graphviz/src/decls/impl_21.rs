macro_rules! deps {
    () => {
        NodeLabels!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl NodeLabels < & 'static str > { fn to_opt_strs (self) -> Vec < Option < & 'static str > > { match self { UnlabelledNodes (len) => vec ! [None ; len] , AllNodesLabelled (lbls) => lbls . into_iter () . map (Some) . collect () , SomeNodesLabelled (lbls) => lbls , } } fn len (& self) -> usize { match self { & UnlabelledNodes (len) => len , & AllNodesLabelled (ref lbls) => lbls . len () , & SomeNodesLabelled (ref lbls) => lbls . len () , } } }
    };
}

impl_21!()