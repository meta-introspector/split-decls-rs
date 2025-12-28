macro_rules! github_event_impl {
    () => {
        # [decl (fn , name = "github_event_impl" , vis = "pub" , hash = "2eb33304")] pub fn github_event_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let repo = input_str . value () ; quote ! { { use std :: process :: Command ; let github_data = Command :: new ("curl") . args (& ["-s" , & format ! ("https://api.github.com/repos/{}" , # repo)]) . output () . map (| o | String :: from_utf8_lossy (& o . stdout) . to_string ()) . unwrap_or_else (| _ | format ! (r#"{{"name":"{}","stars":0}}"# , # repo)) ; let memory_item = format ! ("MemoryItem::GitHubEvent {{ repo: '{}', timestamp: {}, data: '{}' }}" , # repo , std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_secs () , github_data . chars () . take (100) . collect ::< String > ()) ; println ! ("cargo:warning=🐙 GitHub event: {}" , # repo) ; memory_item } } . into () }
    };
}

github_event_impl!()