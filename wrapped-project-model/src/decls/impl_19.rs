macro_rules! deps {
    () => {
        EditionData!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl From < EditionData > for Edition { fn from (data : EditionData) -> Self { match data { EditionData :: Edition2015 => Edition :: Edition2015 , EditionData :: Edition2018 => Edition :: Edition2018 , EditionData :: Edition2021 => Edition :: Edition2021 , EditionData :: Edition2024 => Edition :: Edition2024 , } } }
    };
}

impl_19!();