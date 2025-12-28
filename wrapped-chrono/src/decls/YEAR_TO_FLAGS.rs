macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! YEAR_TO_FLAGS {
    () => {
        deps!();
        const YEAR_TO_FLAGS : & [YearFlags ; 400] = & [BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , C , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , E , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , G , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C , BA , G , F , E , DC , B , A , G , FE , D , C , B , AG , F , E , D , CB , A , G , F , ED , C , B , A , GF , E , D , C ,] ;
    };
}

YEAR_TO_FLAGS!()