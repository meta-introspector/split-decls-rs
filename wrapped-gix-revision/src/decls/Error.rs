macro_rules! deps {
    () => {
        Delegate!();
        Kind!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " The error returned by [`spec::parse()`][crate::spec::parse()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("'~' needs to follow an anchor, like '@~'.")] MissingTildeAnchor , # [error ("':' needs to be followed by either '/' and regex or the path to lookup in the HEAD tree.")] MissingColonSuffix , # [error ("':/' must be followed by a regular expression.")] EmptyTopLevelRegex , # [error ("Need one character after '/!', typically '-', but got {:?}" , . regex)] UnspecifiedRegexModifier { regex : BString } , # [error ("Cannot peel to {:?} - unknown target." , . input)] InvalidObject { input : BString } , # [error ("Could not parse time {:?} for revlog lookup." , . input)] Time { input : BString , source : Option < gix_date :: parse :: Error > , } , # [error ("Sibling branches like 'upstream' or 'push' require a branch name with remote configuration, got {:?}" , . name)] SiblingBranchNeedsBranchName { name : BString } , # [error ("Reflog entries require a ref name, got {:?}" , . name)] ReflogLookupNeedsRefName { name : BString } , # [error ("A reference name must be followed by positive numbers in '@{{n}}', got {:?}" , . nav)] RefnameNeedsPositiveReflogEntries { nav : BString } , # [error ("Negative or explicitly positive numbers are invalid here: {:?}" , . input)] SignedNumber { input : BString } , # [error ("Could not parse number from {input:?}")] InvalidNumber { input : BString } , # [error ("Negative zeroes are invalid: {:?} - remove the '-'" , . input)] NegativeZero { input : BString } , # [error ("The opening brace in {:?} was not matched" , . input)] UnclosedBracePair { input : BString } , # [error ("Cannot set spec kind more than once. Previous value was {:?}, now it is {:?}" , . prev_kind , . kind)] KindSetTwice { prev_kind : spec :: Kind , kind : spec :: Kind } , # [error ("The @ character is either standing alone or followed by `{{<content>}}`, got {:?}" , . input)] AtNeedsCurlyBrackets { input : BString } , # [error ("A portion of the input could not be parsed: {:?}" , . input)] UnconsumedInput { input : BString } , # [error ("The delegate didn't indicate success - check delegate for more information")] Delegate , }
    };
}

Error!();