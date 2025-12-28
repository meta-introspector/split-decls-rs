macro_rules! deps {
    () => {
        Runnable!();
        TestItemKind!();
    };
}

macro_rules! TestItem {
    () => {
        deps!();
        # [derive (Debug)] pub struct TestItem { pub id : String , pub kind : TestItemKind , pub label : String , pub parent : Option < String > , pub file : Option < FileId > , pub text_range : Option < TextRange > , pub runnable : Option < Runnable > , }
    };
}

TestItem!();