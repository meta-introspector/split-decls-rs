macro_rules! deps {
    () => {
        Mode!();
        Location!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        # [doc = " initialization and modification"] impl Mode { # [doc = " Create a mode instance with percentage only."] pub fn with_percentage () -> Self { Mode { percent : true , throughput : false , location : Location :: AfterUnit , } } # [doc = " Create a mode instance with throughput only."] pub fn with_throughput () -> Self { Mode { percent : false , throughput : true , location : Location :: AfterUnit , } } # [doc = " Turn on percentage display on the current instance."] pub fn and_percentage (mut self) -> Self { self . percent = true ; self } # [doc = " Turn on throughput display on the current instance."] pub fn and_throughput (mut self) -> Self { self . throughput = true ; self } # [doc = " Change the display location to show up in front of the value."] pub fn show_before_value (mut self) -> Self { self . location = Location :: BeforeValue ; self } }
    };
}

impl_119!()