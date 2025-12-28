macro_rules! deps {
    () => {
        InlayHintLabelPart!();
        LazyProperty!();
        InlayTooltip!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl fmt :: Debug for InlayHintLabelPart { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self { text , linked_location : None , tooltip : None | Some (LazyProperty :: Lazy) } => { text . fmt (f) } Self { text , linked_location , tooltip } => f . debug_struct ("InlayHintLabelPart") . field ("text" , text) . field ("linked_location" , linked_location) . field ("tooltip" , & tooltip . as_ref () . map_or ("" , | it | match it { LazyProperty :: Computed (InlayTooltip :: String (it) | InlayTooltip :: Markdown (it) ,) => it , LazyProperty :: Lazy => "" , }) ,) . finish () , } } }
    };
}

impl_277!()