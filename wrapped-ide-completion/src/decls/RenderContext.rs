macro_rules! deps {
    () => {
        CompletionContext!();
    };
}

macro_rules! RenderContext {
    () => {
        deps!();
        # [doc = " Interface for data and methods required for items rendering."] # [derive (Debug , Clone)] pub (crate) struct RenderContext < 'a > { completion : & 'a CompletionContext < 'a > , is_private_editable : bool , import_to_add : Option < LocatedImport > , doc_aliases : Vec < SmolStr > , }
    };
}

RenderContext!()