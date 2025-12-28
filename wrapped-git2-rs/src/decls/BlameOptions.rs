macro_rules! deps {
    () => {
        Blame!();
    };
}

macro_rules! BlameOptions {
    () => {
        deps!();
        # [doc = " Blame options"] pub struct BlameOptions { raw : raw :: git_blame_options , }
    };
}

BlameOptions!();