macro_rules! alt_registry_url {
    () => {
        # [doc = " URL to the alternative-registry version of `registry_url`"] fn alt_registry_url () -> Url { generate_url ("alternative-registry") }
    };
}

alt_registry_url!();