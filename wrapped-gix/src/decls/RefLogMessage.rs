macro_rules! deps {
    () => {
        String!();
    };
}

macro_rules! RefLogMessage {
    () => {
        deps!();
        # [doc = " The way reflog messages should be composed whenever a ref is written with recent objects from a remote."] pub enum RefLogMessage { # [doc = " Prefix the log with `action` and generate the typical suffix as `git` would."] Prefixed { # [doc = " The action to use, like `fetch` or `pull`."] action : String , } , # [doc = " Control the entire message, using `message` verbatim."] Override { # [doc = " The complete reflog message."] message : BString , } , }
    };
}

RefLogMessage!()