macro_rules! deps {
    () => {
        Context!();
        Options!();
        State!();
    };
}

macro_rules! Pipeline {
    () => {
        deps!();
        # [doc = " The standard git filter pipeline comprised of multiple standard filters and support for external filters."] # [doc = ""] # [doc = " It's configuring itself for each provided path based on the path's attributes, implementing the complex logic that governs it."] # [derive (Clone)] pub struct Pipeline { # [doc = " Various options that are all defaultable."] options : pipeline :: Options , # [doc = " Storage for the attributes of each item we should process, configured for use with all attributes that concern us."] attrs : gix_attributes :: search :: Outcome , # [doc = " Additional context to pass to process filters."] context : pipeline :: Context , # [doc = " State needed to keep running filter processes."] processes : driver :: State , # [doc = " A utility to handle multiple buffers to keep results of various filters."] bufs : gix_utils :: Buffers , }
    };
}

Pipeline!();