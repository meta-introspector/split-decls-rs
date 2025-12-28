macro_rules! InlayTooltip {
    () => {
        # [derive (Debug , Hash)] pub enum InlayTooltip { String (String) , Markdown (String) , }
    };
}

InlayTooltip!();