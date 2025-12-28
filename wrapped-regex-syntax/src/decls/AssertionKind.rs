macro_rules! AssertionKind {
    () => {
        # [doc = " An assertion kind."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum AssertionKind { # [doc = " `^`"] StartLine , # [doc = " `$`"] EndLine , # [doc = " `\\A`"] StartText , # [doc = " `\\z`"] EndText , # [doc = " `\\b`"] WordBoundary , # [doc = " `\\B`"] NotWordBoundary , # [doc = " `\\b{start}`"] WordBoundaryStart , # [doc = " `\\b{end}`"] WordBoundaryEnd , # [doc = " `\\<` (alias for `\\b{start}`)"] WordBoundaryStartAngle , # [doc = " `\\>` (alias for `\\b{end}`)"] WordBoundaryEndAngle , # [doc = " `\\b{start-half}`"] WordBoundaryStartHalf , # [doc = " `\\b{end-half}`"] WordBoundaryEndHalf , }
    };
}

AssertionKind!()