macro_rules! deps {
    () => {
        HirFrame!();
        Flags!();
        TranslatorBuilder!();
    };
}

macro_rules! Translator {
    () => {
        deps!();
        # [doc = " A translator maps abstract syntax to a high level intermediate"] # [doc = " representation."] # [doc = ""] # [doc = " A translator may be benefit from reuse. That is, a translator can translate"] # [doc = " many abstract syntax trees."] # [doc = ""] # [doc = " A `Translator` can be configured in more detail via a"] # [doc = " [`TranslatorBuilder`]."] # [derive (Clone , Debug)] pub struct Translator { # [doc = " Our call stack, but on the heap."] stack : RefCell < Vec < HirFrame > > , # [doc = " The current flag settings."] flags : Cell < Flags > , # [doc = " Whether we're allowed to produce HIR that can match arbitrary bytes."] utf8 : bool , # [doc = " The line terminator to use for `.`."] line_terminator : u8 , }
    };
}

Translator!()