macro_rules! NodeLabels {
    () => {
        enum NodeLabels < L > { AllNodesLabelled (Vec < L >) , UnlabelledNodes (usize) , SomeNodesLabelled (Vec < Option < L > >) , }
    };
}

NodeLabels!();