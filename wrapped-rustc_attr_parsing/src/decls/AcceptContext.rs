macro_rules! deps {
    () => {
        Stage!();
        SharedContext!();
    };
}

macro_rules! AcceptContext {
    () => {
        deps!();
        # [doc = " Context given to every attribute parser when accepting"] # [doc = ""] # [doc = " Gives [`AttributeParser`]s enough information to create errors, for example."] pub struct AcceptContext < 'f , 'sess , S : Stage > { pub (crate) shared : SharedContext < 'f , 'sess , S > , # [doc = " The span of the attribute currently being parsed"] pub (crate) attr_span : Span , # [doc = " Whether it is an inner or outer attribute"] pub (crate) attr_style : AttrStyle , # [doc = " The expected structure of the attribute."] # [doc = ""] # [doc = " Used in reporting errors to give a hint to users what the attribute *should* look like."] pub (crate) template : & 'f AttributeTemplate , # [doc = " The name of the attribute we're currently accepting."] pub (crate) attr_path : AttrPath , }
    };
}

AcceptContext!();