macro_rules! WriteStyle {
    () => {
        # [doc = " Whether or not to print styles to the target."] # [allow (clippy :: exhaustive_enums)] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , Default)] pub enum WriteStyle { # [doc = " Try to print styles, but don't force the issue."] # [default] Auto , # [doc = " Try very hard to print styles."] Always , # [doc = " Never print styles."] Never , }
    };
}

WriteStyle!();