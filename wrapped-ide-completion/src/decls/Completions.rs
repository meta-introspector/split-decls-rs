macro_rules! deps {
    () => {
        CompletionItem!();
    };
}

macro_rules! Completions {
    () => {
        deps!();
        # [doc = " Represents an in-progress set of completions being built."] # [derive (Debug , Default)] pub struct Completions { buf : Vec < CompletionItem > , }
    };
}

Completions!();