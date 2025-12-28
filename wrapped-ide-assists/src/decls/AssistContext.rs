macro_rules! deps {
    () => {
        AssistConfig!();
        Assists!();
    };
}

macro_rules! AssistContext {
    () => {
        deps!();
        # [doc = " `AssistContext` allows to apply an assist or check if it could be applied."] # [doc = ""] # [doc = " Assists use a somewhat over-engineered approach, given the current needs."] # [doc = " The assists workflow consists of two phases. In the first phase, a user asks"] # [doc = " for the list of available assists. In the second phase, the user picks a"] # [doc = " particular assist and it gets applied."] # [doc = ""] # [doc = " There are two peculiarities here:"] # [doc = ""] # [doc = " * first, we ideally avoid computing more things then necessary to answer \"is"] # [doc = "   assist applicable\" in the first phase."] # [doc = " * second, when we are applying assist, we don't have a guarantee that there"] # [doc = "   weren't any changes between the point when user asked for assists and when"] # [doc = "   they applied a particular assist. So, when applying assist, we need to do"] # [doc = "   all the checks from scratch."] # [doc = ""] # [doc = " To avoid repeating the same code twice for both \"check\" and \"apply\""] # [doc = " functions, we use an approach reminiscent of that of Django's function based"] # [doc = " views dealing with forms. Each assist receives a runtime parameter,"] # [doc = " `resolve`. It first check if an edit is applicable (potentially computing"] # [doc = " info required to compute the actual edit). If it is applicable, and"] # [doc = " `resolve` is `true`, it then computes the actual edit."] # [doc = ""] # [doc = " So, to implement the original assists workflow, we can first apply each edit"] # [doc = " with `resolve = false`, and then applying the selected edit again, with"] # [doc = " `resolve = true` this time."] # [doc = ""] # [doc = " Note, however, that we don't actually use such two-phase logic at the"] # [doc = " moment, because the LSP API is pretty awkward in this place, and it's much"] # [doc = " easier to just compute the edit eagerly :-)"] pub (crate) struct AssistContext < 'a > { pub (crate) config : & 'a AssistConfig , pub (crate) sema : Semantics < 'a , RootDatabase > , frange : FileRange , trimmed_range : TextRange , source_file : SourceFile , token_at_offset : TokenAtOffset < SyntaxToken > , covering_element : SyntaxElement , }
    };
}

AssistContext!()