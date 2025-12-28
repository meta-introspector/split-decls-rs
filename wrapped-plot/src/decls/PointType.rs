macro_rules! PointType {
    () => {
        # [doc = " Point type"] # [allow (missing_docs)] # [derive (Clone , Copy)] pub enum PointType { Circle , FilledCircle , FilledSquare , FilledTriangle , Plus , Square , Star , Triangle , X , }
    };
}

PointType!()