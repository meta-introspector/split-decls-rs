macro_rules! deps {
    () => {
        AssistId!();
        GroupLabel!();
        SourceChange!();
        Label!();
        Command!();
    };
}

macro_rules! Assist {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct Assist { pub id : AssistId , # [doc = " Short description of the assist, as shown in the UI."] pub label : Label , pub group : Option < GroupLabel > , # [doc = " Target ranges are used to sort assists: the smaller the target range,"] # [doc = " the more specific assist is, and so it should be sorted first."] pub target : TextRange , # [doc = " Computing source change sometimes is much more costly then computing the"] # [doc = " other fields. Additionally, the actual change is not required to show"] # [doc = " the lightbulb UI, it only is needed when the user tries to apply an"] # [doc = " assist. So, we compute it lazily: the API allow requesting assists with"] # [doc = " or without source change. We could (and in fact, used to) distinguish"] # [doc = " between resolved and unresolved assists at the type level, but this is"] # [doc = " cumbersome, especially if you want to embed an assist into another data"] # [doc = " structure, such as a diagnostic."] pub source_change : Option < SourceChange > , # [doc = " The command to execute after the assist is applied."] pub command : Option < Command > , }
    };
}

Assist!()