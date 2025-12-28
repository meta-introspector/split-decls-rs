macro_rules! deps {
    () => {
        Round!();
        Float!();
        StatusAnd!();
    };
}

macro_rules! FloatConvert {
    () => {
        deps!();
        # [doc = " Convert between floating point types."] pub trait FloatConvert < T : Float > : Float { # [doc = " Convert a value of one floating point type to another."] # [doc = " The return value corresponds to the IEEE754 exceptions. *loses_info"] # [doc = " records whether the transformation lost information, i.e. whether"] # [doc = " converting the result back to the original type will produce the"] # [doc = " original value (this is almost the same as return value==Status::OK,"] # [doc = " but there are edge cases where this is not so)."] fn convert_r (self , round : Round , loses_info : & mut bool) -> StatusAnd < T > ; # [doc = " Convert with default [`NearestTiesToEven`](Round::NearestTiesToEven) rounding."] fn convert (self , loses_info : & mut bool) -> StatusAnd < T > { self . convert_r (Round :: NearestTiesToEven , loses_info) } }
    };
}

FloatConvert!()