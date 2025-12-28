macro_rules! VZip {
    () => {
        # [doc = " Combine single vectors into a multi-lane vector."] pub trait VZip < V > { fn vzip (self) -> V ; }
    };
}

VZip!();