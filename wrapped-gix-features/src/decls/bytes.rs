macro_rules! bytes {
    () => {
        # [doc = " A unit for displaying bytes with throughput and progress percentage."] # [cfg (not (feature = "progress-unit-bytes"))] pub fn bytes () -> Option < Unit > { Some (unit :: label_and_mode ("B" , unit :: display :: Mode :: with_throughput () . and_percentage () ,)) }
    };
}

bytes!()