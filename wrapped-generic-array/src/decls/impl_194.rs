macro_rules! deps {
    () => {
        LengthError!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        # [rustversion :: since (1.81)] impl core :: error :: Error for LengthError { }
    };
}

impl_194!()