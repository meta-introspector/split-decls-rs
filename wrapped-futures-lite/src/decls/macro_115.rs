macro_rules! macro_115 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::flat_map()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct FlatMap < S , U , F > { # [pin] stream : Map < S , F >, # [pin] inner_stream : Option < U >, } }
    };
}

macro_115!()