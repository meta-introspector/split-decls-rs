macro_rules! deps {
    () => {
        Item!();
        Duration!();
    };
}

macro_rules! ticker {
    () => {
        deps!();
        # [doc = " Returns a stream of 'ticks', each being duration `dur` apart."] # [doc = ""] # [doc = " Can be useful to provide the TUI with additional events in regular intervals,"] # [doc = " when using the [`tui::render_with_input(…events)`](./fn.render_with_input.html) function."] pub fn ticker (dur : Duration) -> impl futures_core :: Stream < Item = () > { let mut delay = Timer :: after (dur) ; futures_lite :: stream :: poll_fn (move | ctx | { let res = Pin :: new (& mut delay) . poll (ctx) ; match res { Poll :: Pending => Poll :: Pending , Poll :: Ready (_) => { delay = Timer :: after (dur) ; Poll :: Ready (Some (())) } } }) }
    };
}

ticker!();