macro_rules! CguReuse {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , PartialOrd)] pub enum CguReuse { No , PreLto , PostLto , }
    };
}

CguReuse!()