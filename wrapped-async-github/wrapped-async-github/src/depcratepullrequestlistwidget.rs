// Generated macro for PullRequestListWidget (struct)
macro_rules! DepcratePullRequestListWidget {
() => {
// Module: crate
// Provides: {"PullRequestListWidget"}
// Dependencies: {}
# [doc = " A widget that displays a list of pull requests."] # [doc = ""] # [doc = " This is an async widget that fetches the list of pull requests from the GitHub API. It contains"] # [doc = " an inner `Arc<RwLock<PullRequestListState>>` that holds the state of the widget. Cloning the"] # [doc = " widget will clone the Arc, so you can pass it around to other threads, and this is used to spawn"] # [doc = " a background task to fetch the pull requests."] # [derive (Debug , Clone , Default)] struct PullRequestListWidget { state : Arc < RwLock < PullRequestListState > > , }
};
}
